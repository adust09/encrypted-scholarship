"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useAccount } from "wagmi";
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
import { CheckCircle, XCircle, AlertCircle } from "lucide-react";

type FeedbackType = "success" | "error" | "warning" | null;

interface FeedbackMessage {
  type: FeedbackType;
  title: string;
  message: string;
}

export default function ScholarshipForm() {
  const [bankBalance, setBankBalance] = useState("");
  const [gpa, setGpa] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errors, setErrors] = useState<{ gpa?: string; bankBalance?: string }>(
    {}
  );
  const [feedback, setFeedback] = useState<FeedbackMessage | null>(null);
  const { address, isConnected } = useAccount();
  const router = useRouter();

  const showFeedback = (type: FeedbackType, title: string, message: string) => {
    setFeedback({ type, title, message });
    // Auto-hide success messages after 5 seconds
    if (type === "success") {
      setTimeout(() => setFeedback(null), 5000);
    }
  };

  const clearFeedback = () => {
    setFeedback(null);
  };

  const validateForm = () => {
    const newErrors: { gpa?: string; bankBalance?: string } = {};

    // Validate GPA (maximum 5.0)
    const gpaValue = parseFloat(gpa);
    if (isNaN(gpaValue) || gpaValue < 0 || gpaValue > 5) {
      newErrors.gpa = "GPA must be between 0 and 5.0";
    }

    // Validate bank balance
    const balanceValue = parseFloat(bankBalance);
    if (isNaN(balanceValue) || balanceValue < 0) {
      newErrors.bankBalance = "Bank balance must be a positive number";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    clearFeedback();

    if (!isConnected) {
      showFeedback(
        "warning",
        "Wallet Not Connected",
        "Please connect your wallet first to submit an application."
      );
      return;
    }

    if (!validateForm()) {
      showFeedback(
        "error",
        "Validation Error",
        "Please fix the form errors before submitting."
      );
      return;
    }

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
          walletAddress: address,
        }),
      });

      if (response.ok) {
        const result = await response.json();
        console.log("Evaluation result:", result);

        showFeedback(
          "success",
          "Application Submitted Successfully!",
          `Your scholarship application has been processed. ${
            result.approved
              ? "Congratulations! You are eligible for the scholarship."
              : "Unfortunately, you do not meet the current criteria."
          }`
        );

        // Redirect after showing success message
        setTimeout(() => {
          router.push(
            `/result?approved=${result.approved}&bankBalance=${bankBalance}&gpa=${gpa}&wallet=${address}`
          );
        }, 2000);
      } else {
        const errorData = await response.json().catch(() => ({}));
        showFeedback(
          "error",
          "Submission Failed",
          errorData.message ||
            "Failed to process your application. Please try again."
        );
      }
    } catch (error) {
      console.error("Error submitting form:", error);
      showFeedback(
        "error",
        "Network Error",
        "Unable to connect to the server. Please check your internet connection and try again."
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleGpaChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setGpa(value);

    // Clear error when user starts typing
    if (errors.gpa) {
      setErrors((prev) => ({ ...prev, gpa: undefined }));
    }
    // Clear feedback when user makes changes
    if (feedback) {
      clearFeedback();
    }
  };

  const handleBankBalanceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setBankBalance(value);

    // Clear error when user starts typing
    if (errors.bankBalance) {
      setErrors((prev) => ({ ...prev, bankBalance: undefined }));
    }
    // Clear feedback when user makes changes
    if (feedback) {
      clearFeedback();
    }
  };

  const getFeedbackIcon = (type: FeedbackType) => {
    switch (type) {
      case "success":
        return <CheckCircle className="h-5 w-5 text-green-500" />;
      case "error":
        return <XCircle className="h-5 w-5 text-red-500" />;
      case "warning":
        return <AlertCircle className="h-5 w-5 text-yellow-500" />;
      default:
        return null;
    }
  };

  const getFeedbackStyles = (type: FeedbackType) => {
    switch (type) {
      case "success":
        return "bg-green-50 border-green-200 text-green-800";
      case "error":
        return "bg-red-50 border-red-200 text-red-800";
      case "warning":
        return "bg-yellow-50 border-yellow-200 text-yellow-800";
      default:
        return "";
    }
  };

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle>Application Form</CardTitle>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          {/* Feedback Message */}
          {feedback && (
            <div
              className={`p-4 border rounded-md ${getFeedbackStyles(
                feedback.type
              )}`}
            >
              <div className="flex items-start space-x-3">
                {getFeedbackIcon(feedback.type)}
                <div className="flex-1">
                  <h4 className="font-medium text-sm">{feedback.title}</h4>
                  <p className="text-sm mt-1">{feedback.message}</p>
                </div>
                <button
                  type="button"
                  onClick={clearFeedback}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <XCircle className="h-4 w-4" />
                </button>
              </div>
            </div>
          )}

          {/* Wallet Connection Warning */}
          {!isConnected && (
            <div className="p-3 bg-yellow-50 border border-yellow-200 rounded-md">
              <div className="flex items-center space-x-2">
                <AlertCircle className="h-4 w-4 text-yellow-500" />
                <p className="text-sm text-yellow-800">
                  Please connect your wallet above to submit an application.
                </p>
              </div>
            </div>
          )}

          <div className="space-y-2">
            <Label htmlFor="bankBalance">Bank Balance ($)</Label>
            <Input
              id="bankBalance"
              type="number"
              value={bankBalance}
              onChange={handleBankBalanceChange}
              required
              placeholder="100,000"
              min="0"
              disabled={!isConnected}
            />
            {errors.bankBalance && (
              <p className="text-sm text-red-500">{errors.bankBalance}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="gpa">GPA (Max: 5.0)</Label>
            <Input
              id="gpa"
              type="number"
              value={gpa}
              onChange={handleGpaChange}
              required
              step="0.01"
              min="0"
              max="5"
              placeholder="3.5"
              disabled={!isConnected}
            />
            {errors.gpa && <p className="text-sm text-red-500">{errors.gpa}</p>}
          </div>
        </CardContent>
        <CardFooter>
          <Button
            type="submit"
            className="w-full"
            disabled={isSubmitting || !isConnected}
          >
            {isSubmitting ? "Processing..." : "Apply for Scholarship"}
          </Button>
        </CardFooter>
      </form>
    </Card>
  );
}
