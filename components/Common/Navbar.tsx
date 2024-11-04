// components/Navbar.tsx
"use client"
import Link from "next/link";
import  ConnectWallet  from "./ConnectWallet";


const Navbar: React.FC = () => {
  return (

<nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
      <Link href="/" className="text-white font-bold text-xl">
            OmniBet
          </Link>

        <div className="flex space-x-4">
          <Link href="/" className="my-2 text-gray-300 hover:text-white transition">
            Predict
          </Link>
          <Link href="/CreatePool" className="my-2 text-gray-300 hover:text-white transition">
            Create Pool
          </Link>
          <div className="relative group">
      <button className="my-2 text-gray-300 hover:text-white transition">
        Get Pool
      </button>
      <div className="absolute hidden group-hover:block w-48 bg-gray-800 rounded-md shadow-lg py-1 z-50 top-full">
        <Link href="/GetAllPool" className="block px-4 py-2 text-sm text-gray-300 hover:text-white hover:bg-gray-700">
          Get All Pools
        </Link>
        <Link href="/GetPoolByToken" className="block px-4 py-2 text-sm text-gray-300 hover:text-white hover:bg-gray-700">
          Get Pools By Token
        </Link>
        <Link href="/GetPoolByDate" className="block px-4 py-2 text-sm text-gray-300 hover:text-white hover:bg-gray-700">
          Get Pools By Date
        </Link>
      </div>
    </div>
          <ConnectWallet />
        </div>
      </div>
    </nav>

    
  );
};

export default Navbar;
