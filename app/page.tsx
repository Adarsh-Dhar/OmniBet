import Image from "next/image";
import Navbar from "@/components/Navbar";
import Predict from "@/components/Predict";



export default function Home() {
  return (
    <div>
    <Navbar />
    <Predict />
    </div>
  );
}
