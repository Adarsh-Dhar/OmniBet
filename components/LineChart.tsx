"use client";
import { useRef, useEffect } from "react";
import { Chart, ChartData, ChartOptions } from "chart.js/auto";

const LineChart: React.FC = () => {
  const chartRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    if (chartRef.current) {
      // If a chart instance already exists, destroy it before creating a new one
      if ((chartRef.current as any).chart) {
        (chartRef.current as any).chart.destroy();
      }

      const context = chartRef.current.getContext("2d");
      if (!context) return; // Ensure context is not null

      // Define the chart data and options
      const data: ChartData<'line'> = {
        labels: [15, 37, 43, 47, 57, 76, 113],
        datasets: [
          {
            label: "Info",
            data: [34, 64, 23, 45, 67, 24, 64],
            backgroundColor: "rgba(255, 99, 132, 0.2)",
            borderColor: "rgb(255, 99, 132)",
            borderWidth: 1,
          },
        ],
      };

      const options: ChartOptions<'line'> = {
        scales: {
          x: {
            type: "linear",
          },
          y: {
            beginAtZero: true,
          },
        },
      };

      const newChart = new Chart(context, {
        type: "line",
        data,
        options,
      });

      // Attach the chart instance to the ref for cleanup on unmount
      (chartRef.current as any).chart = newChart;
    }
  }, []);

  return (
    <div style={{ position: "relative", width: "90vw", height: "80vh" }}>
      <canvas ref={chartRef} />
    </div>
  );
};

export default LineChart;
