import React from "react";


import ChatsDetailsLayout from "../components/layouts/ChatsDetailsLayout";
import ChatsDetails from "../components/sections/ChatsDetails";


export default function ChatsDetailsPage({ isSignedIn, wallet }) {
  return (
    <ChatsDetailsLayout isSignedIn={isSignedIn} wallet={wallet}>
        <ChatsDetails />
    </ChatsDetailsLayout>
  );
}