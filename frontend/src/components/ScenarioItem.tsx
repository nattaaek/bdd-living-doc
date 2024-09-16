// src/components/ScenarioItem.tsx
import React, { useState } from 'react';
import StepItem from './StepItem';

interface Step {
  keyword: string;
  value: string;
}

interface Scenario {
  name: string;
  steps: Step[];
}

interface ScenarioItemProps {
  scenario: Scenario;
}

function ScenarioItem({ scenario }: ScenarioItemProps) {
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const toggleSteps = () => {
    setIsOpen(!isOpen);
  };

  return (
    <div className="border rounded mb-4">
      <button
        onClick={toggleSteps}
        className="w-full flex justify-between items-center p-4 bg-gray-100 hover:bg-gray-200"
      >
        <span className="font-semibold">{scenario.name}</span>
        {isOpen ? (
          <svg
            className="h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            {/* Up Arrow Icon */}
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 15l7-7 7 7" />
          </svg>
        ) : (
          <svg
            className="h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            {/* Down Arrow Icon */}
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        )}
      </button>
      {isOpen && (
        <div className="p-4">
          {scenario.steps.map((step, index) => (
            <StepItem key={index} step={step} />
          ))}
        </div>
      )}
    </div>
  );
}

export default ScenarioItem;
