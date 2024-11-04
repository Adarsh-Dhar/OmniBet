import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import GetAllPool from "@/components/Bet/GetAllPool";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetAllPool />
    </div>
  );
}
