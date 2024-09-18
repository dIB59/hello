import * as React from "react";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export function AIAgentChat() {
  return (
    <Card className="w-[750px] h-[500px] flex flex-col">
      <CardHeader className="pb-2">
        <CardTitle>Chat with your AI Agent</CardTitle>
        <CardDescription>
          Describe the scenario and interact with your AI agent.
        </CardDescription>
      </CardHeader>

      {/* Chat messages section */}
      <div className="flex-1 overflow-y-auto p-4 bg-gray-50 border rounded-md">
        {/* Example chat bubbles */}
        <div className="mb-2">
          <div className="bg-blue-500 text-white p-3 rounded-lg max-w-[70%]">
            Hello! How can I assist you today?
          </div>
        </div>
        <div className="mb-2 text-right">
          <div className="bg-gray-200 text-black p-3 rounded-lg max-w-[70%] ml-auto">
            I need help with my project.
          </div>
        </div>
      </div>

      {/* Input area */}
      <div className="p-4 border-t">
        <div className="flex items-center gap-2">
          <Textarea
            className="flex-1 h-10"
            placeholder="Type your message here."
          />
          <Button className="w-[150px]">Send message</Button>
        </div>
      </div>
    </Card>
  );
}

export default AIAgentChat;
