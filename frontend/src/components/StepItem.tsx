// src/components/StepItem.tsx
import React from 'react';

interface Step {
  keyword: string;
  value: string;
}

interface StepItemProps {
  step: Step;
}

function StepItem({ step }: StepItemProps) {
  return (
    <div className="pl-4 mb-2">
      <p>
        <span className="font-semibold">{step.keyword}</span> {step.value}
      </p>
    </div>
  );
}

export default StepItem;
