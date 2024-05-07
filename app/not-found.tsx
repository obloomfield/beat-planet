import Link from "next/link";

export default function NotFound() {
  return (
    <div className="flex flex-col items-center">
      <h2 className="font-bold text-4xl text-center">
        <span className="drop-shadow-[0_5px_10px_rgba(255,50,50,0.7)]">
          404
        </span>
        : Not Found 🫠
      </h2>
      <Link className="hover:underline text-center p-2" href="/maps">
        Return Home
      </Link>
    </div>
  );
}
