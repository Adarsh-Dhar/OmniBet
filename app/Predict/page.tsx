import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import PredictionGraph from "@/components/Graphs/PredictionGraph";
import Bet from "@/components/Bet/Bet";




export default function Home() {
  return (
    <div>
    <Navbar />
    <PredictionGraph />
    <Bet />
    </div>
  );
}
