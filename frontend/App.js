import 'regenerator-runtime/runtime';
import React from 'react';
import './assets/global.css';
import {Route,Routes} from 'react-router-dom'
import Landing from "./pages/Landing";
import FooterCrypto from "./components/sections/Footer"
import Profile from './pages/Profile';
import Jobs from "./pages/Jobs";
import Chats from "./pages/Chats";
import Transactions from "./pages/Transactions"
import JobDetailsPage from "./pages/JobDetails"
import ChatsDetailsPage from './pages/ChatsDetailsPage';




export default function App({ isSignedIn, contractId, wallet }) {
  const [valueFromBlockchain, setValueFromBlockchain] = React.useState();

  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  const [jobs, setJobs] = React.useState([]);
  // Get blockchian state once on component load
  React.useEffect(() => {
    getAllClientJobs().then(setJobs);
    }
  , []);




  async function getAllClientJobs() {
    try {

      const res = await this.wallet.viewMethod({
        method: 'get_all_client_jobs',
        contractId,
      });
  
      return res; // Return the result if it's successful
  
    } catch (error) {
      console.error('An error occurred while fetching client jobs:', error);
      // Handle the error gracefully, e.g., by logging it or notifying the user
      return { error: 'Failed to fetch client jobs' }; // Return an error object
    }
  }
  
  

  console.log(jobs)


  return (
    <>
      <div className='container'>

        <Routes>
          {/* <Route path="/" element = {<Home />} /> */}
          <Route path="/" element = {<Landing isSignedIn={isSignedIn} wallet={wallet}/>} />
          <Route path="/freelancer" element = {<Profile isSignedIn={isSignedIn} wallet={wallet}/>} />
          <Route path="/jobs" element = {<Jobs isSignedIn={isSignedIn} wallet={wallet} contractId={contractId}/>} />
          <Route path="/chats" element = {<Chats isSignedIn={isSignedIn} wallet={wallet}/>} />
          <Route path="/transactions" element = {<Transactions isSignedIn={isSignedIn} wallet={wallet}/>} />
          <Route path="/job/:id" element = {<JobDetailsPage isSignedIn={isSignedIn} wallet={wallet} contractId={contractId}/>} />
          <Route path="/chat/:id" element = {<ChatsDetailsPage wallet={wallet} isSignedIn={isSignedIn}/>} />
        </Routes>

      </div>
      <FooterCrypto />

    </>
  );
}