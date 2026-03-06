import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Backend } from "../target/types/backend";
import { expect } from "chai";

describe("backend program - full lifecycle", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Backend as Program<Backend>;
  const user = provider.wallet;

  let configPda: anchor.web3.PublicKey;
  let profilePda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;

  const EASY_PRICE = new anchor.BN(1_000_000);

  before(async () => {
    // CONFIG PDA
    [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeConfig(EASY_PRICE, EASY_PRICE, EASY_PRICE)
      .accountsPartial({
        authority: user.publicKey,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // PROFILE PDA
    [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    // VAULT PDA
    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .createProfile()
      .accountsPartial({
        user: user.publicKey,
        profile: profilePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  // ---------------------------------------
  // PROFILE INITIALIZATION
  // ---------------------------------------

  it("Profile initializes correctly", async () => {
    const profile = await program.account.userProfile.fetch(profilePda);

    expect(profile.successStreak).to.equal(0);
    expect(profile.failureStreak).to.equal(0);
    expect(profile.stakeMultiplier.toNumber()).to.equal(10000);
  });

  // ---------------------------------------
  // CREATE TASK
  // ---------------------------------------

  it("Creates task successfully", async () => {
    const profile = await program.account.userProfile.fetch(profilePda);

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        user.publicKey.toBuffer(),
        new anchor.BN(profile.activeTasks)
          .toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const deadline = new anchor.BN(
      Math.floor(Date.now() / 1000) + 60
    );

    await program.methods
      .createTask(
        "Test Task",
        "Testing success flow",
        { easy: {} },
        deadline
      )
      .accountsPartial({
        user: user.publicKey,
        profile: profilePda,
        task: taskPda,
        vault: vaultPda,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const task = await program.account.task.fetch(taskPda);

    expect(task.status.active).to.be.true;
  });

  // ---------------------------------------
  // COMPLETE TASK SUCCESS
  // ---------------------------------------

  it("Completes task and resets multiplier", async () => {
    const profileBefore = await program.account.userProfile.fetch(profilePda);

    const taskIndex = profileBefore.activeTasks - 1;

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        user.publicKey.toBuffer(),
        new anchor.BN(taskIndex)
          .toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    await program.methods
      .completeTask()
      .accountsPartial({
        user: user.publicKey,
        profile: profilePda,
        task: taskPda,
        vault: vaultPda,
      })
      .rpc();

    const profileAfter = await program.account.userProfile.fetch(profilePda);

    expect(profileAfter.successStreak).to.equal(1);
    expect(profileAfter.failureStreak).to.equal(0);
    expect(profileAfter.stakeMultiplier.toNumber()).to.equal(10000);
  });

  // ---------------------------------------
  // FAIL TASK FLOW
  // ---------------------------------------

  it("Fails task and increases multiplier", async () => {
    const profile = await program.account.userProfile.fetch(profilePda);

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        user.publicKey.toBuffer(),
        new anchor.BN(profile.activeTasks)
          .toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const deadline = new anchor.BN(
      Math.floor(Date.now() / 1000) + 2
    );

    await program.methods
      .createTask(
        "Fail Task",
        "Testing failure",
        { easy: {} },
        deadline
      )
      .accountsPartial({
        user: user.publicKey,
        profile: profilePda,
        task: taskPda,
        vault: vaultPda,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // wait for deadline
    await new Promise((r) => setTimeout(r, 3000));

    await program.methods
      .markFailed()
      .accountsPartial({
        user: user.publicKey,
        profile: profilePda,
        task: taskPda,
      })
      .rpc();

    const updatedProfile = await program.account.userProfile.fetch(profilePda);

    expect(updatedProfile.failureStreak).to.equal(1);
    expect(updatedProfile.successStreak).to.equal(0);
    expect(updatedProfile.stakeMultiplier.toNumber()).to.equal(12500);
  });

  // ---------------------------------------
  // PUNISHMENT LOCK CHECK
  // ---------------------------------------

  it("Cannot claim punishment before 24h", async () => {
    const profile = await program.account.userProfile.fetch(profilePda);

    const taskIndex = profile.activeTasks - 1;

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        user.publicKey.toBuffer(),
        new anchor.BN(taskIndex)
          .toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    try {
      await program.methods
        .completePunishment()
        .accountsPartial({
          user: user.publicKey,
          profile: profilePda,
          task: taskPda,
          vault: vaultPda,
        })
        .rpc();

      expect.fail("Should have thrown PunishmentNotOver");
    } catch (err: any) {
      expect(err.error.errorCode.code).to.equal("PunishmentNotOver");
    }
  });
});