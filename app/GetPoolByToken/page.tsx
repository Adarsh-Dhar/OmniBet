import Image from "next/image";
import Navbar from "@/components/Common/Navbar";
import GetPoolByToken from "@/components/Bet/GetPoolByToken";

export default function Home() {
  return (
    <div>
    <Navbar />
    <GetPoolByToken />
    </div>
  );
}
