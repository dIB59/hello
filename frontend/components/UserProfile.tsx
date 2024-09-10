import React from "react";

type UserProfileProps = {
  name: string;
  password: string;
};

const UserProfile: React.FC<UserProfileProps> = ({ name, password }) => {
  return (
    <div className="max-w-md mx-auto bg-white shadow-md rounded-lg overflow-hidden">
      <div className="flex items-center p-6">
        <div>
          <h2 className="text-xl font-semibold text-gray-800">{name}</h2>
          <p className="text-gray-600">{password}</p>
        </div>
      </div>
    </div>
  );
};

export default UserProfile;
