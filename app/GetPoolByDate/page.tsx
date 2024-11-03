import Image from "next/image";
import Navbar from "@/components/Navbar";
import GetPoolByDate from "@/components/GetPoolByDate";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetPoolByDate />
    </div>
  );
}
