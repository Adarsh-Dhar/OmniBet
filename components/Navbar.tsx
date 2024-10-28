// components/Navbar.tsx
import Link from "next/link";
import Button from "./Button";
import ConnectWallet from "./ConnectWallet";

const Navbar: React.FC = () => {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
      <Link href="/" className="text-white font-bold text-xl">
            OmniBet
          </Link>
        <div className="flex space-x-4">
          
          <ConnectWallet />
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
