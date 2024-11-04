import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import Predict from "@/components/Pool/Predict";
import LineChart from "@/components/Graphs/LineChart";



export default function Home() {
  return (
    <div>
    <Navbar />
    <Predict />
    </div>
  );
}
