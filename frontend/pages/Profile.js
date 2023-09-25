import React from "react";


import ProfileLayout from "../components/layouts/ProfileLayout";
import ProfileTabs from "../components/sections/ProfileTabs";


export default function Profile({ isSignedIn, wallet }) {
  return (
    <ProfileLayout isSignedIn={isSignedIn} wallet={wallet}>
        <ProfileTabs />
    </ProfileLayout>
  );
}