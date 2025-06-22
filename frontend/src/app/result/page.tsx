"use client";

import { useSearchParams } from "next/navigation";
import { Suspense } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { CheckCircle, XCircle, ArrowLeft } from "lucide-react";
import Link from "next/link";

function ResultContent() {
  const searchParams = useSearchParams();
  const approved = searchParams.get("approved") === "true";
  const bankBalance = searchParams.get("bankBalance");
  const gpa = searchParams.get("gpa");
  const wallet = searchParams.get("wallet");

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <Card className="w-full max-w-md mx-auto">
        <CardHeader className="text-center">
          <div className="flex justify-center mb-4">
            {approved ? (
              <CheckCircle className="h-16 w-16 text-green-500" />
            ) : (
              <XCircle className="h-16 w-16 text-red-500" />
            )}
          </div>
          <CardTitle className={`text-2xl ${approved ? "text-green-700" : "text-red-700"}`}>
            {approved ? "Congratulations!" : "Application Result"}
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className={`p-4 rounded-lg ${approved ? "bg-green-50 border border-green-200" : "bg-red-50 border border-red-200"}`}>
            <p className={`text-center font-medium ${approved ? "text-green-800" : "text-red-800"}`}>
              {approved 
                ? "Your scholarship application has been approved! You are eligible for funding."
                : "Unfortunately, your application does not meet the current eligibility criteria."
              }
            </p>
          </div>

          <div className="space-y-3 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-600">Bank Balance:</span>
              <span className="font-medium">${bankBalance ? Number(bankBalance).toLocaleString() : "N/A"}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">GPA:</span>
              <span className="font-medium">{gpa || "N/A"}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Wallet:</span>
              <span className="font-mono text-xs break-all">{wallet || "N/A"}</span>
            </div>
          </div>

          <div className="pt-4 border-t">
            <p className="text-xs text-gray-500 text-center mb-4">
              Eligibility criteria: Bank balance &lt; $50,000 and GPA ≥ 3.0
            </p>
            <Link href="/" className="block">
              <Button className="w-full" variant="outline">
                <ArrowLeft className="h-4 w-4 mr-2" />
                Submit Another Application
              </Button>
            </Link>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

export default function ResultPage() {
  return (
    <Suspense fallback={
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">Loading...</div>
      </div>
    }>
      <ResultContent />
    </Suspense>
  );
}