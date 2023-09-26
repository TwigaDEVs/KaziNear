import React from "react";

import JobsDetailsLayout from "../components/layouts/JobsDetailsLayout";
import JobDetails from "../components/sections/JobDetails";

export default function JobsDetailsPage({ isSignedIn, wallet }) {
  return (
    <JobsDetailsLayout isSignedIn={isSignedIn} wallet={wallet}>
        <JobDetails />
    </JobsDetailsLayout>
  );
}