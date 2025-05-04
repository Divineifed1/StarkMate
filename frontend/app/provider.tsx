"use client";
import React, {  useEffect, } from "react";
import { useConnect, useAccount } from "@starknet-react/core";

export function Providers({ children }: { children: React.ReactNode }) {
  const { connectors, connectAsync } = useConnect({});
  const {status,address}=useAccount()
  // const { address, status, connector } = useAccount();

  // const [connecting, setConnecting] = useState(true)
  useEffect(() => {
    const tryReconnect = async () => {
      const LS_connector = localStorage.getItem("connector");
      if (!LS_connector) return;
  
      const connector = connectors.find((con) => con.id === LS_connector);
      if (!connector) return;
  
      try {
        if (status === "disconnected") {
          await connectAsync({ connector });
          console.log("Connected successfully!");
        }
      } catch (err) {
        console.error("Connection error:", err);
      }
  
      console.log("Status:", status, "Address:", address);
    };
  
    tryReconnect();
  }, [address, status,connectAsync,connectors]);
  

  return <>{children}</>;
}
