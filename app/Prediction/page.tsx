import Image from "next/image";
import Navbar from "@/components/Navbar";
import PredictionGraph from "@/components/PredictionGraph";




export default function Home() {
  return (
    <div>
    <Navbar />
    <PredictionGraph />
    </div>
  );
}
