"use client";
import { useRef, useEffect, useState } from "react";
import { Chart, ChartData, ChartOptions } from "chart.js/auto";
import { usePriceStore } from "@/states/state";


const LineChart: React.FC = () => {
  const chartRef = useRef<HTMLCanvasElement | null>(null);
  const [chartData, setChartData] = useState<ChartData<'line'>>({
    labels: [],
    datasets: [
      {
        label: 'My First Dataset',
        data: [],
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
      },
    ],
  });

  useEffect(() => {
    const fetchAndUpdateData = async () => {

      const prices = usePriceStore((state) => state.prices);
      const dates = usePriceStore((state) => state.dates);
      setChartData({
        labels: dates,
        datasets: [
          {
            label: 'My First Dataset',
            data: prices,
            fill: false,
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1,
          },
        ],
      });
    };

    fetchAndUpdateData();
  }, []);

  useEffect(() => {
    if (chartRef.current) {
      // If a chart instance already exists, destroy it before creating a new one
      if ((chartRef.current as any).chart) {
        (chartRef.current as any).chart.destroy();
      }

      const context = chartRef.current.getContext("2d");
      if (!context) return; // Ensure context is not null

      const newChart = new Chart(context, {
        type: "line",
        data: chartData,
        options: {
          scales: {
            x: {
              type: "linear",
            },
          },
        },
      });

      // Attach the chart instance to the ref for cleanup on unmount
      (chartRef.current as any).chart = newChart;
    }
  }, [chartData]);

  return (
    <div style={{ position: "relative", width: "90vw", height: "80vh" }}>
      <canvas ref={chartRef} />
    </div>
  );
};

export default LineChart;