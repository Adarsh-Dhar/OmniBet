// components/Navbar.tsx
import Link from "next/link";
import Button from "./Button";

const Navbar: React.FC = () => {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <h1 className="text-white font-bold text-xl">My App</h1>
        <div className="flex space-x-4">
          <Link href="/borrow" className="text-gray-300 hover:text-white transition">
            Borrow
          </Link>
          <Link href="/repay" className="text-gray-300 hover:text-white transition">
            Repay
          </Link>
          <Button />
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
