import React from "react";




import JobsLayout from "../components/layouts/JobsLayout";
import JobList from "../components/sections/JobsList";


export default function Jobs({ isSignedIn, wallet }) {
  return (
    <JobsLayout isSignedIn={isSignedIn} wallet={wallet}>
        <JobList />
    </JobsLayout>
  );
}