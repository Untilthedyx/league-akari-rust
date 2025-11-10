import { Loader2 } from "lucide-react";
import { useInitStore } from "@/lib/store/initStore";

export default function InitScreen() {
  const initMessage = useInitStore((state) => state.initMessage);

  return (
    <div className="flex items-center justify-center h-screen w-screen bg-background">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="size-8 animate-spin text-primary" />
        <p className="text-muted-foreground">{initMessage}</p>
      </div>
    </div>
  );
}