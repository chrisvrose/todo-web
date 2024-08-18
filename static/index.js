console.log("Started");
const listcontainer = document.querySelector(".todocontainer");

fetch("/todo")
  .then((e) => e.json())
  .then((todolist) => {
    todolist.forEach((todo) => {
      const ul = document.createElement("li");
      ul.innerText = todo.title;
      ul.setAttribute('data-index', todo.id)
      ul.setAttribute('data-state', todo.state)
      listcontainer.appendChild(ul);

    });
  })
  .catch(console.error);
