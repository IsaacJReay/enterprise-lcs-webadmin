import React from "react";
import { Table } from "antd";
import { FiFileText } from "react-icons/fi";

const LocalTable = () => {
  const columns = [
    {
      title: "Name",
      dataIndex: "info",
      key: "info",
      render: (info) => {
        {
          info.map((res) => {
            return (
              <React.Fragment>
                {res.type === "file" && (
                  <div>
                    <FiFileText />
                    {res.name}
                  </div>
                )}
              </React.Fragment>
            );
          });
        }
      },
    },
    {
      title: "Date",
      dataIndex: "date",
    },
    {
      title: "Size",
      dataIndex: "size",
    },
  ];
  const data = [
    {
      type: "file",
      info: {
        name: "index.php",
        type: "file",
      },
      date: "2021-07-04 22:07:15",
      size: "12.5M",
    },
  ];
  return (
    <React.Fragment>
      <Table rowSelection columns={columns} dataSource={data} />
    </React.Fragment>
  );
};

export default LocalTable;
