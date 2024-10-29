import Image from "next/image";
import Navbar from "@/components/Navbar";
import Predict from "@/components/Predict";
import LineChart from "@/components/LineChart";



export default function Home() {
  return (
    <div>
    <Navbar />
    <Predict />
    </div>
  );
}
