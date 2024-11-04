import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import GetPoolByDate from "@/components/Bet/GetPoolByDate";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetPoolByDate />
    </div>
  );
}
