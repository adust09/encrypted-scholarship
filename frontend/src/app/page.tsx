import ScholarshipForm from "../components/scholarship-form";
import { WalletConnect } from "../components/wallet-connect";

export default function Home() {
  return (
    <main className="container mx-auto px-4 py-8">
      <h1 className="text-2xl font-bold text-center mb-6">
        Encrypted Scholarship
      </h1>
      <WalletConnect />
      <ScholarshipForm />
    </main>
  );
}
