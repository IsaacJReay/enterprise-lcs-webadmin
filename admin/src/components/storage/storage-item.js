import React, { useState } from "react";
import { FcFolder, FcFile } from "react-icons/fc";
import { Row, Col } from "antd";

export default function StorageItem({
  data,
  parent,
  setParent,
  setSelected,
  selected,
}) {
  const { name } = data;
  const { size } = data.meta;
  const [expand, setExpand] = useState(false);

  function formatBytes(size, decimals = 2) {
    if (size === 0) return "0 B";
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    const total = Math.floor(Math.log(size) / Math.log(k));
    return (
      parseFloat((size / Math.pow(k, total)).toFixed(dm)) + " " + sizes[total]
    );
  }

  if (data.meta.is_dir) {
    const selfLocation = `${parent}${name}/`;
    return (
      <ul className="file-container">
        <li
          className={`list-file ${
            selected === selfLocation ? "selected-file" : ""
          }`}
          onClick={(e) => {
            setExpand(!expand);
            setSelected(selfLocation);
            console.log(`${selfLocation} is dir`);
          }}
        >
          <Row gutter={6}>
            <Col span={2}>
              <FcFolder size={35} className="folder-icon" />
            </Col>
            <Col span={20}>
              <p className="file-name">{name}</p>
            </Col>
            <Col span={2}>
              {" "}
              <div className="file-size">{formatBytes(size)}</div>
            </Col>
          </Row>
        </li>

        {!expand ? (
          ""
        ) : (
          <ul>
            {data.children.map((child) => (
              <StorageItem
                data={child}
                parent={selfLocation}
                setParent={setParent}
                setSelected={setSelected}
                selected={selected}
              />
            ))}
          </ul>
        )}
      </ul>
    );
  } else {
    const selfLocation = `${parent}${name}`;
    return (
      <li
        className={`list-file ${
          selected === selfLocation ? "selected-file" : ""
        }`}
        onClick={(e) => {
          setSelected(selfLocation);
          console.log(`${selfLocation} is file`);
        }}
      >
        <Row gutter={12}>
          <Col span={2}>
            <FcFile size={35} className="folder-icon" />
          </Col>
          <Col span={20}>{name}</Col>
          <Col span={2}>
            <div>{formatBytes(size)}</div>
          </Col>
        </Row>
      </li>
    );
  }
}
