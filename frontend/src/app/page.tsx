import ScholarshipForm from "../components/scholarship-form";

export default function Home() {
  return (
    <main className="container mx-auto px-4 py-8">
      <h1 className="text-2xl font-bold text-center mb-6">
        奨学金審査システム
      </h1>
      <ScholarshipForm />
    </main>
  );
}
