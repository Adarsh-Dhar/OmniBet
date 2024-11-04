import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import PredictionGraph from "@/components/Graphs/PredictionGraph";
import CreatePredictionPool from "@/components/Pool/CreatePool";




export default function Home() {
  return (
    <div>
    <Navbar />
    <PredictionGraph />
    <CreatePredictionPool />
    </div>
  );
}
