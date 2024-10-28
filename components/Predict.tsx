import React, { useState } from 'react';
import ChainList from './ChainList'; // Adjust this import based on your file structure

const Predict: React.FC = () => {
  const [primaryToken, setPrimaryToken] = useState<string | null>(null);
  const [referenceToken, setReferenceToken] = useState<string | null>(null);

  const handlePredict = () => {
    if (primaryToken && referenceToken) {
      // Handle prediction logic here (e.g., sending data to backend)
      console.log(`Predicting ${primaryToken} against ${referenceToken}`);
    } else {
      alert('Please select both a primary and a reference token.');
    }
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Make a Prediction</h1>
      
      <div className="mb-4">
        <h2 className="text-lg font-semibold mb-2">Select Primary Token</h2>
        <ChainList />
      </div>

      <div className="mb-4">
        <h2 className="text-lg font-semibold mb-2">Select Reference Token</h2>
        <ChainList />
      </div>

      <button
        onClick={handlePredict}
        className="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
      >
        Predict
      </button>
    </div>
  );
};

export default Predict;
