import { createAppKit } from "@reown/appkit/react";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { mainnet, arbitrum, polygon } from "@wagmi/core/chains";

// 1. Get projectId from https://cloud.reown.com
const projectId = process.env.NEXT_PUBLIC_REOWN_PROJECT_ID || "your-project-id";

// 2. Create a metadata object - optional
const metadata = {
  name: "Encrypted Scholarship",
  description:
    "Privacy-preserving scholarship application using zero-knowledge proofs",
  url: "https://encrypted-scholarship.app", // origin must match your domain & subdomain
  icons: ["https://avatars.githubusercontent.com/u/179229932"],
};

// 3. Set the networks
const networks = [mainnet, arbitrum, polygon];

// 4. Create Wagmi Adapter
const wagmiAdapter = new WagmiAdapter({
  networks,
  projectId,
  ssr: true,
});

// 5. Create modal
export const modal = createAppKit({
  adapters: [wagmiAdapter],
  networks,
  projectId,
  metadata,
  features: {
    analytics: true, // Optional - defaults to your Cloud configuration
  },
});

export { wagmiAdapter };
