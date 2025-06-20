"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";

export default function ScholarshipForm() {
  const [bankBalance, setBankBalance] = useState("");
  const [gpa, setGpa] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      const response = await fetch("/api/evaluate-scholarship", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          bankBalance: parseFloat(bankBalance),
          gpa: parseFloat(gpa),
        }),
      });

      if (response.ok) {
        const result = await response.json();
        console.log("Evaluation result:", result);
        router.push(
          `/result?approved=${result.approved}&bankBalance=${bankBalance}&gpa=${gpa}`
        );
      } else {
        console.error("Evaluation failed");
      }
    } catch (error) {
      console.error("Error submitting form:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle>Application Form</CardTitle>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="bankBalance">Bank Balance ($)</Label>
            <Input
              id="bankBalance"
              type="number"
              value={bankBalance}
              onChange={(e) => setBankBalance(e.target.value)}
              required
              placeholder="100,000"
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="gpa">GPA</Label>
            <Input
              id="gpa"
              type="number"
              value={gpa}
              onChange={(e) => setGpa(e.target.value)}
              required
              step="0.01"
              min="0"
              max="4"
              placeholder="3.5"
            />
          </div>
        </CardContent>
        <CardFooter>
          <Button type="submit" className="w-full" disabled={isSubmitting}>
            {isSubmitting ? "processing..." : "Apply for Scholarship"}
          </Button>
        </CardFooter>
      </form>
    </Card>
  );
}
