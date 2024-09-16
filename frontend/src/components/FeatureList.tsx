// src/components/FeatureList.tsx
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import FeatureItem from './FeatureItem';

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

function FeatureList() {
  const [features, setFeatures] = useState<Feature[]>([]);

  useEffect(() => {
    const fetchFeatures = async () => {
      try {
        const response = await axios.get<Feature[]>('http://localhost:8080/features');
        setFeatures(response.data);
      } catch (error) {
        console.error('Error fetching features:', error);
      }
    };

    fetchFeatures();

    const socket = new WebSocket('ws://localhost:8080/ws/');

    socket.onmessage = (event) => {
      if (event.data === 'update') {
        fetchFeatures();
      }
    };

    return () => {
      socket.close();
    };
  }, []);

  return (
    <>
      {features.map((feature, index) => (
        <FeatureItem key={index} feature={feature} />
      ))}
    </>
  );
}

export default FeatureList;
