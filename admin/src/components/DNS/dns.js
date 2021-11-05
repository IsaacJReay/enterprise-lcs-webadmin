import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Form,
  Table,
  Checkbox,
  Tag,
  message,
  Popconfirm,
  Divider,
  Button,
} from "antd";

import { Link } from "react-router-dom";
import CreateDomain from "./create-domain";
import axios from "axios";

const { Content } = Layout;

const DNSSetting = () => {
  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };
  // --------state ---------

  const [loading, setLoading] = useState(false);
  const [datas, setDatas] = useState([]);

  // ----------get data -------------

  useEffect(async () => {
    await axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/dns/domain_name/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setLoading(true);
        setDatas(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  //  --------delete record ----------

  const handleDelete = (id) => {
    axios
      .delete(
        "http://10.42.0.188:8080/private/api/settings/dns/domain_name/deletion",
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
          data: { id: id },
        }
      )
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  const columns = [
    {
      title: "No",
      width: "10%",
      dataIndex: "id",
      key: "id",
    },
    {
      title: "Name",
      dataIndex: "domain_name",
      key: "domain_name",
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
      render: (status) => {
        return <Checkbox disabled checked={status} />;
      },
    },
    {
      title: "Actions",
      dataIndex: "id",
      render: (id, data) => {
        const { domain_name } = data;
        return (
          <React.Fragment>
            <Link to={`/dns-management/${id}`}>
              <Tag color="processing">Manage</Tag>
            </Link>
            <Divider type="vertical" />
            <Popconfirm
              placement="top"
              title={
                <span>
                  Are you sure to delete <b>{domain_name} </b>?
                </span>
              }
              okText="Yes"
              cancelText="No"
              onConfirm={() => handleDelete(id)}
            >
              <Tag color="error" style={{ cursor: "pointer" }}>
                Delete
              </Tag>
            </Popconfirm>
          </React.Fragment>
        );
      },
    },
  ];
  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>DNS Setting</h1>
              </div>
              <hr />

              <div className="dns-desc-container">
                <Form.Item>
                  <CreateDomain />
                </Form.Item>
              </div>
              <Form>
                <div className="dns-desc-container">
                  <Form.Item>
                    <Table
                      columns={columns}
                      dataSource={datas}
                      pagination={false}
                      scroll={{ y: 450 }}
                    />
                  </Form.Item>
                </div>
              </Form>
            </div>
          </Col>
          <Col span={8}>
            <div className="container">
              <div className="container-header">
                <h1>Desciptions</h1>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default DNSSetting;
