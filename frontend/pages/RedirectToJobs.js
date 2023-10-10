import React, { useEffect } from 'react';
import { useLocation, useHistory } from 'react-router-dom';

function RedirectToJobs() {
  const location = useLocation();
  const history = useHistory();

  useEffect(() => {
    // Check if the "transactionHashes" query parameter is present in the URL
    const params = new URLSearchParams(location.search);
    if (params.has('transactionHashes')) {
      // Redirect to the home page when the parameter is present
      history.push('/jobs');
    }
  }, [location.search, history]);

  return null; // This component doesn't render anything
}

export default RedirectToJobs;
