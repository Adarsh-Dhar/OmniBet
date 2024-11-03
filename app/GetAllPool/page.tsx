import Image from "next/image";
import Navbar from "@/components/Navbar";
import GetAllPool from "@/components/GetAllPool";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetAllPool />
    </div>
  );
}
