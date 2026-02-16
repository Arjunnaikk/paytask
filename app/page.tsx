"use client";
import { useWalletConnection } from "@solana/react-hooks";
import { VaultCard } from "./components/vault-card";

export default function Home() {
  const { connectors, connect, disconnect, wallet, status } =
    useWalletConnection();

  const address = wallet?.account.address.toString();

  return(
    <>
    
    </>
  );
}
