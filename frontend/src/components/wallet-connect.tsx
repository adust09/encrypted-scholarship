"use client";

import { useAppKit } from "@reown/appkit/react";
import { useAccount, useDisconnect } from "wagmi";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function WalletConnect() {
  const { open } = useAppKit();
  const { address, isConnected } = useAccount();
  const { disconnect } = useDisconnect();

  const handleConnect = () => {
    open();
  };

  const handleDisconnect = () => {
    disconnect();
  };

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  return (
    <Card className="w-full max-w-md mx-auto mb-6">
      <CardHeader>
        <CardTitle>Wallet Connection</CardTitle>
      </CardHeader>
      <CardContent>
        {isConnected ? (
          <div className="space-y-4">
            <div className="text-sm text-gray-600">
              Connected to:{" "}
              <span className="font-mono">{formatAddress(address!)}</span>
            </div>
            <Button
              onClick={handleDisconnect}
              variant="outline"
              className="w-full"
            >
              Disconnect Wallet
            </Button>
          </div>
        ) : (
          <div className="space-y-4">
            <p className="text-sm text-gray-600">
              Connect your wallet to submit scholarship applications
            </p>
            <Button onClick={handleConnect} className="w-full">
              Connect Wallet
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
