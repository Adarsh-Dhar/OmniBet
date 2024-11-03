import Image from "next/image";
import Navbar from "@/components/Navbar";
import GetPoolByToken from "@/components/GetPoolByToken";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetPoolByToken />
    </div>
  );
}
