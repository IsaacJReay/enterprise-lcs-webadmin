import React from "react";
import { Table, Tag } from "antd";
import data from "../api/data.json";

const DownloadHosting = () => {
  const columns = [
    {
      title: "No",
      dataIndex: "id",
      key: "id",
      width: "10%",
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Actions",
      dataIndex: "id",
      width: "20%",
      render: () => {
        return (
          <React.Fragment>
            <Tag color="processing" style={{ cursor: "pointer" }}>
              Update
            </Tag>
            <Tag color="error" style={{ cursor: "pointer" }}>
              Disable
            </Tag>
          </React.Fragment>
        );
      },
    },
  ];
  return (
    <div>
      <React.Fragment>
        <Table
          columns={columns}
          dataSource={data}
          pagination={false}
          scroll={{ y: 450 }}
        />
      </React.Fragment>
    </div>
  );
};

export default DownloadHosting;
