// components/CreatePredictionPool.tsx
"use client"
import { useState } from 'react';
import { useStore } from '../states/state';
import { useTransaction } from '../interaction/useTransaction';

const CreatePredictionPool = () => {
  const [predictionDate, setPredictionDate] = useState('');
  const [deadlineDate, setDeadlineDate] = useState('');

  const [entryFee, setEntryFee] = useState<number | ''>('');
  const token = useStore((state : any) => state.tokenSelected)
  const owner = useStore((state : any) => state.address)
  const { createPool } = useTransaction();

  // Handle form submission
  const handleCreatePool = (e: React.FormEvent) => {
    e.preventDefault();

    // Calculate deadline in seconds
    const today = new Date();
    const startDate = Math.floor((today.getTime())/ 1000).toString();
    console.log("start date", startDate)                     
    const predictDate = new Date(predictionDate);
    const endDate = Math.floor((predictDate.getTime()) / 1000).toString();
    console.log("end date", endDate);
    const deadline = new Date(deadlineDate);
    const deadlineTimestamp = Math.floor(deadline.getTime() / 1000).toString();

    // TODO: Send data to the backend or store it in the app state
    console.log('Pool created:', { endDate, entryFee, deadlineTimestamp });
    createPool(owner, startDate, endDate, token, entryFee?.toString() || "0", deadlineTimestamp);
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow-md max-w-lg mx-auto mt-10">
      <h2 className="text-2xl font-semibold mb-4 text-gray-800">Create a Prediction Pool</h2>
      
        <div>
          <label htmlFor="predictionDate" className="block text-sm font-medium text-gray-700">
            Prediction Date
          </label>
          <input
            type="date"
            id="predictionDate"
            value={predictionDate}
            onChange={(e) => setPredictionDate(e.target.value)}
            className="mt-1 p-2 border border-gray-300 rounded-md w-full"
            required
          />
        </div>
        <div>
          
          <input
            type="number"
            id="entryFee"
            value={entryFee}
            onChange={(e) => setEntryFee(e.target.valueAsNumber || '')}
            className="mt-1 p-2 border border-gray-300 rounded-md w-full"
            placeholder="0"
          />
        </div>
        <div>
          <label htmlFor="predictionDate" className="block text-sm font-medium text-gray-700">
            Prediction Deadline
          </label>
          <input
            type="date"
            id="deadlineDate"
            value={deadlineDate}
            onChange={(e) => setDeadlineDate(e.target.value)}
            className="mt-1 p-2 border border-gray-300 rounded-md w-full"
            required
          />
        </div>
        <button
          type="submit"
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition"  onClick={handleCreatePool}
        >
          Create Pool
        </button>
      
    </div>
  );
};

export default CreatePredictionPool;
