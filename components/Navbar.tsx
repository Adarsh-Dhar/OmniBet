// components/Navbar.tsx
import Link from "next/link";
import Button from "./Button";
import ConnectWallet from "./ConnectWallet";

const Navbar: React.FC = () => {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <h1 className="text-white font-bold text-xl">My App</h1>
        <div className="flex space-x-4">
          
          <ConnectWallet />
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
