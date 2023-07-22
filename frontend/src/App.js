import { useState, useEffect } from 'react';
function ProductCategoryRow({ category }) {
  return (
    <tr>
      <th colSpan="2">
        {category}
      </th>
    </tr>
  );
}

function ProductRow({ product }) {
  const name = product.stocked ? product.name :
    <span style={{ color: 'red' }}>
      {product.name}
    </span>;

  return (
    <tr>
      <td>{name}</td>
      <td>{product.price}</td>
    </tr>
  );
}

function ProductTable({ products }) {
  const rows = [];
  let lastCategory = null;

  products.forEach((product) => {
    if (product.category !== lastCategory) {
      rows.push(
        <ProductCategoryRow
          category={product.category}
          key={product.category} />
      );
    }
    rows.push(
      <ProductRow
        product={product}
        key={product.name} />
    );
    lastCategory = product.category;
  });

  return (
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Price</th>
        </tr>
      </thead>
      <tbody>{rows}</tbody>
    </table>
  );
}
async function sendRequest(file_type, settingFunction) {
  const requestOptions = {
    method: 'POST',
    headers: { 'Conetent-Type': 'application/json' },
    body: JSON.stringify({ filetype: file_type })
  };
  console.log(requestOptions);
  await fetch('/fetch_by_filetype', requestOptions)
    .then(response => response.json())
    .then(data => settingFunction(data));

}
function SearchBar() {
  const [text, setText] = useState("");
  const [respResult, setRespResult] = useState({});
  const [submitted, setSubmitted] = useState('');

  function handleChange(e) {
    setText(e.target.value);
    console.log(text);
  }

  function handleSubmit(e) {
    e.preventDefault();
    setSubmitted(text);
    console.log('Sent');
    console.log(text);
    sendRequest(text, setRespResult);
    // console.log(respResult);
    setText("");
  }
  // useEffect(() => {
  //   sendRequest()
  //     .then((res) => {
  //       setRespResult(res)
  //     })
  //     .catch((e) => {
  //       console.log(e.message)
  //     })
  // }, [])
  return (
    <div>
      <a>{respResult !== {} && respResult.cursor}</a>
      <ul>
        {(typeof respResult.matching_files !== 'undefined') && respResult.matching_files.map(item => (
          <li>{`https://arweave.net/${item}`}</li>
        ))}
      </ul>
      <form onSubmit={handleSubmit}>
        <input type="text" onChange={handleChange} placeholder="search..." />
      </form>
    </div>
  );
}

function FilterableProductTable({ products }) {
  return (
    <div>
      <SearchBar />
      <ProductTable products={products} />
    </div>
  );
}

const PRODUCTS = [
  { category: "Fruits", price: "$1", stocked: true, name: "Apple" },
  { category: "Fruits", price: "$1", stocked: true, name: "Dragonfruit" },
  { category: "Fruits", price: "$2", stocked: false, name: "Passionfruit" },
  { category: "Vegetables", price: "$2", stocked: true, name: "Spinach" },
  { category: "Vegetables", price: "$4", stocked: false, name: "Pumpkin" },
  { category: "Vegetables", price: "$1", stocked: true, name: "Peas" }
];

export default function App() {
  return <FilterableProductTable products={PRODUCTS} />;
}
