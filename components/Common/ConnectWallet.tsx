"use client"
import { useState } from "react";
import { useStore } from "@/states/state";
import { NibiruTxClient, Testnet } from "@nibiruchain/nibijs";
import { Button } from "../ui/button"


interface KeplrWindow {
  keplr?: {
    enable: (chainId: string) => Promise<void>;
    getOfflineSigner: (chainId: string) => {
      getAccounts: () => Promise<{ address: string; }[]>;
    };
  };
}

declare global {
  interface Window extends KeplrWindow {}
}

const WalletConnect = () => {

  const [error, setError] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const userAddress = useStore((state : any) => state.address);
  const updateAddress = useStore((state : any) => state.changeAddress);
  const changeOfflineSigner = useStore((state : any) => state.changeOfflineSigner);

  


  // Function to truncate address for display
  const truncateAddress = (addr: string) => {
    if (!addr) return "";
    const start = addr.slice(0, 8);
    const end = addr.slice(-6);
    return `${start}...${end}`;
  };

  const connectWallet = async () => {
    setIsConnecting(true);
    setError(null);
    try {
      const chainId = "nibiru-testnet-1";
      if (!window.keplr) {
        throw new Error("Keplr wallet not found! Please install Keplr extension.");
      }
      
      await window.keplr.enable(chainId);
      const offlineSigner = window.keplr.getOfflineSigner(chainId);
      changeOfflineSigner(offlineSigner);
      const accounts = await offlineSigner.getAccounts();
      
      if (!accounts || accounts.length === 0) {
        throw new Error("No accounts found");
      }

      
      updateAddress(accounts[0].address);
      console.log("user address", userAddress);
    } catch (err: any) {
      setError(err.message || "Failed to connect to wallet");
    } finally {
      setIsConnecting(false);
    }
  };

  return (

      <div className="flex flex-col items-center gap-4">
     

      <Button
        onClick={connectWallet}
        disabled={isConnecting}
      
      >
        <span className="relative px-5 py-2.5 transition-all ease-in duration-75 rounded-md group-hover:bg-opacity-0">
          {isConnecting ? (
            "Connecting..."
          ) : userAddress ? (
            truncateAddress(userAddress)
          ) : (
            "Connect Wallet"
          )}
        </span>
      </Button>
      
      
      {error && (
        <p className="text-red-500 text-sm">{error}</p>
      )}
    </div>

    
  );
};

export default WalletConnect;