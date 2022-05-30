import React, { useState } from "react";
import { Row, Col } from "antd";
import { MoreOutlined } from "@ant-design/icons";
import { FcFolder, FcFile } from "react-icons/fc";

export default function StorageItem({
  data,
  parent,
  setParent,
  setSelected,
  selected,
}) {
  // ===========state============
  const { name, meta } = data;
  const { item_size, item_last_modify_date } = meta;
  const [expand, setExpand] = useState(false);

  function formatBytes(item_size, decimals = 2) {
    if (item_size === 0) return "0 B";
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    const total = Math.floor(Math.log(item_size) / Math.log(k));
    return (
      parseFloat((item_size / Math.pow(k, total)).toFixed(dm)) +
      " " +
      sizes[total]
    );
  }

  if (data.meta.item_is_dir) {
    const selfLocation = `${parent}${name}/`;
    return (
      <>
        <ul className="file-container">
          <li
            className={`list-file ${
              selected === selfLocation ? "selected-file" : ""
            }`}
            onClick={(e) => {
              e.preventDefault();
              setExpand(!expand);
              setSelected(selfLocation);
              // console.log(`${selfLocation} is dir`);
            }}
          >
            <Row gutter={6}>
              <Col span={2}>
                <FcFolder size={35} className="folder-icon" />
              </Col>
              <Col span={11}>
                <p className="file-name">{name}</p>
              </Col>
              <Col span={8}>
                <p className="file-name">{item_last_modify_date}</p>
              </Col>
              <Col span={2}>
                <div className="file-size">{formatBytes(item_size)}</div>
              </Col>
              <Col span={1}>
                <MoreOutlined className="more-storage-icon" />
              </Col>
            </Row>
          </li>

          {!expand ? (
            ""
          ) : (
            <ul className="children-dir">
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
      </>
    );
  } else {
    const selfLocation = `${parent}${name}`;
    return (
      <>
        <li
          className={`list-file ${
            selected === selfLocation ? "selected-file" : ""
          }`}
          onClick={(e) => {
            setSelected(selfLocation);
            // console.log(`${selfLocation} is file`);
          }}
        >
          <Row gutter={12}>
            <Col span={2}>
              <FcFile size={35} className="folder-icon" />
            </Col>
            <Col span={11}>
              <p className="filename">{name}</p>
            </Col>
            <Col span={8}>
              <p className="file-name">{item_last_modify_date}</p>
            </Col>
            <Col span={2}>
              <p className="filename">{formatBytes(item_size)}</p>
            </Col>
            <Col span={1}>
              <MoreOutlined className="more-storage-icon" />
            </Col>
          </Row>
        </li>
      </>
    );
  }
}
