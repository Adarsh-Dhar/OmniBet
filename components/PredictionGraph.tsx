import { LineController, LineElement, PointElement, CategoryScale, LinearScale } from "chart.js";

const PredictionGraph = () => {

const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
const data = {
  labels: labels,
  datasets: [{
    label: 'My First Dataset',
    data: [65, 59, 80, 81, 56, 55, 40],
    fill: false,
    borderColor: 'rgb(75, 192, 192)',
    tension: 0.1
  }]
};

const config = {
    type: 'line',
    data: data,
  };

  return(
    <div>
        
    </div>
  )
}

export default PredictionGraph;