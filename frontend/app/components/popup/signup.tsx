import React from 'react';
import styles from './RegistrationForm.module.css';

type RegistrationFormProps = {
  addTask?: (task: { taskName: string; assignee: string }) => void; // Example prop if needed
};

const RegistrationForm: React.FC<RegistrationFormProps> = ({ addTask }) => {
  return (
    <div className={styles.registrationCard}>
      <div className={styles.header}>
        <h1>
          Let’s get <span className={styles.highlight}>you started</span>, superstar! 🚀
        </h1>
        <p>What will <strong>you</strong> be creating today?</p>
      </div>

      <div className={styles.buttonContainer}>
        <button className={styles.signUpButtonGoogle}>
          Sign up with Google <span className={styles.googleIcon}>G</span>
        </button>
        <button className={styles.signUpButtonApple}>
          Sign up with Apple <span className={styles.appleIcon}></span>
        </button>
      </div>

      <form className={styles.form}>
        <div className={styles.row}>
          <div className={styles.inputContainer}>
            <label className={styles.label}>First name:</label>
            <input type="text" placeholder="First name" className={styles.inputField} />
          </div>
          <div className={styles.inputContainer}>
            <label className={styles.label}>Surname:</label>
            <input type="text" placeholder="Surname" className={styles.inputField} />
          </div>
        </div>
        <div className={styles.inputContainer}>
          <label className={styles.label}>
            Business name: <span className={styles.optional}>(optional)</span>
          </label>
          <input type="text" placeholder="Business name" className={styles.inputField} />
        </div>
        <div className={styles.inputContainer}>
          <label className={styles.label}>Email address:</label>
          <input type="email" placeholder="Email address" className={styles.inputField} />
        </div>
        <div className={styles.inputContainer}>
          <label className={styles.label}>Choose password:</label>
          <input type="password" placeholder="Password" className={styles.inputField} />
        </div>
        <p className={styles.passwordInfo}>
          For your own security, your chosen password must include at least: One uppercase letter, One lowercase letter,
          One number, One special character.
        </p>
      </form>
    </div>
  );
};

export default RegistrationForm;

