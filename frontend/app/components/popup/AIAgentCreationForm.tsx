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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export function CardWithForm() {
  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>Create AI Agent</CardTitle>
        <CardDescription>
          Deploy your new AI Agent in one-click.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5">
              <Label htmlFor="name">AI Agent Name</Label>
              <Input id="name" placeholder="Name of your AI Agent" />
            </div>
            <div className="flex flex-col space-y-1.5">
              <Label htmlFor="Type">Agent Type</Label>
              <Select>
                <SelectTrigger id="Type">
                  <SelectValue placeholder="Select" />
                </SelectTrigger>
                <SelectContent position="popper">
                  <SelectItem value="LLM">LLM</SelectItem>
                  <SelectItem value="Image">Image generation</SelectItem>
                  <SelectItem value="Video">Video Generation</SelectItem>
                  <SelectItem value="Audio">Audio Generation</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </form>
      </CardContent>
      <CardFooter className="flex justify-between">
        <Button variant="outline">Cancel</Button>
        <Button>Create</Button>
      </CardFooter>
    </Card>
  );
}
