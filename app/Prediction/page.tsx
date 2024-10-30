import Image from "next/image";
import Navbar from "@/components/Navbar";
import PredictionGraph from "@/components/PredictionGraph";
import CreatePredictionPool from "@/components/CreatePool";




export default function Home() {
  return (
    <div>
    <Navbar />
    <PredictionGraph />
    <CreatePredictionPool />
    </div>
  );
}
