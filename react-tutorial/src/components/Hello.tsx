// const name = "Seth";

// function displayMessage() {
//   return "Welcome to React!";
// }

function Hello({
  name,
  message,
  emoji,
}: {
  name: string;
  message: string;
  emoji: string;
}) {
  //   console.log("name = ", props.name);
  //   console.log("message = ", props.message);
  //   console.log(props);
  // const {name, message} = props;

  // JSX renders both html and evaluate javascript expressions. We can use curly braces to evaluate javascript expressions in JSX.
  // name = "guys";
  // prop.name = "guys";
  return (
    <div>
      {/* <h1> */}
      {/* {displayMessage()} <br></br>
            Hello from {name}. Who is {10 + 10} years old.
        </h1>

        <h1>Another element</h1> */}
      Hello from {name}. {message}. {emoji}
    </div>
  );
}

export default Hello;

// Checkpoint: passing arrays and objects as props
// https://www.youtube.com/watch?v=KU-I2M9Jm68&list=PLSsAz5wf2lkK_ekd0J__44KG6QoXetZza&index=12