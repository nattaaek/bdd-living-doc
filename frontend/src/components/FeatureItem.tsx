// src/components/FeatureItem.tsx
import React from 'react';
import ScenarioItem from './ScenarioItem';

interface Step {
  keyword: string;
  value: string;
}

interface Scenario {
  name: string;
  steps: Step[];
}

interface Feature {
  name: string;
  description?: string;
  scenarios: Scenario[];
}

interface FeatureItemProps {
  feature: Feature;
}

function FeatureItem({ feature }: FeatureItemProps) {
  return (
    <div className="bg-white shadow rounded mb-6 p-6">
      <h2 className="text-2xl font-bold mb-2">{feature.name}</h2>
      {feature.description && <p className="mb-4">{feature.description}</p>}
      <div>
        {feature.scenarios.map((scenario, index) => (
          <ScenarioItem key={index} scenario={scenario} />
        ))}
      </div>
    </div>
  );
}

export default FeatureItem;
